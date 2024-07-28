//
//  SignView.swift
//  MeetlyIOS
//
//  Created by Niko Neufeld on 24.07.2024.
//

import SwiftUI
import AuthenticationServices
import Alamofire

struct SigninView: View {
    @State var signInCompleted = false
    @State var jwtToken = ""
    var body: some View {
        //TODO: LOAD OLD REFRESH TOKEN AND BYPASS SIGNIN
        Group {
            if !signInCompleted {
                VStack {
                    Text("Log in!").font(.title)
                    SignInWithAppleButton(.signIn) { request in
                        // authorization request for an Apple ID
                        request.requestedScopes = [.email, .fullName]
                    } onCompletion: { result in
                        // completion handler that is called when the sign-in completes
                        switch result {
                        case .success(let authorization):
                            if let userCredential = authorization.credential as? ASAuthorizationAppleIDCredential {
                                print("Signing in!")
                                print(userCredential.user)
                                print(userCredential.authorizedScopes)
                                let jwtToken = String(data: userCredential.identityToken!,encoding: .utf8)!
                                let givenName = (userCredential.fullName ?? nil)?.givenName ?? nil
                                let familyName = (userCredential.fullName ?? nil)?.familyName ?? nil
                                let email = (userCredential.email ?? nil)
                                let signInWithJwt = LogInWithJwt(jwt: jwtToken, givenName: givenName,user: userCredential.user, familyName: familyName,email: email)
                                AF.request("http://10.0.1.145:3000/signin_apple",
                                           method: .post,
                                           parameters: signInWithJwt,
                                           encoder: JSONParameterEncoder.default).responseDecodable(of:AppleIDSignInResponse.self) { response in
                                    if let resp = response.response {
                                        if resp.statusCode == 201 {
                                            debugPrint(response)
                                            if case let .success(r) = response.result {
                                                do {
                                                    try KeychainItem(service: "com.mast3rsoft.MeetlyIOSAPP", account: "refreshToken").saveItem(r.refresh_token)
                                                } catch {
                                                    fatalError("Unable to save userIdentifier to keychain.")
                                                }
                                                withAnimation {
                                                    self.jwtToken = r.server_jwt
                                                    signInCompleted = true
                                                }
                                            }
                                            
                                        }
                                    }
                                }
                                
                                
                                // print(userCredential.identityToken)
                                
                            }
                        case .failure(let error):
                            print("Could not authenticate: \\(error.localizedDescription)")
                        }
                        
                    }.font(.headline).frame(height:50).padding()
                }
            } else {
                HomePage(jwtToken:jwtToken )
            }
        }.onAppear {
            
            do {
                var refreshToken = try KeychainItem(service: "com.mast3rsoft.MeetlyIOSAPP", account: "refreshToken").readItem()
                var refreshTokenStruct = RefreshToken(refresh_token: refreshToken)
                AF.request("http://10.0.1.145:3000/refresh_token",
                           method: .post,
                           parameters: refreshTokenStruct,
                           encoder: JSONParameterEncoder.default).responseDecodable(of:RefreshTokenResponse.self) { response in
                        if let resp = response.response {
                            if resp.statusCode == 201 {
                                if case let .success(jwtToken) = response.result {
                                    //  withAnimation {
                                        self.jwtToken = jwtToken.server_jwk_token
                                        signInCompleted = true
                                  //  }
                                }
                            }
                        }
                    }
            } catch {
                print("grrr")
                // do nothing - continue to sign in
                
            }
            
        }
        
    }
}
#Preview {
    SigninView()
}
