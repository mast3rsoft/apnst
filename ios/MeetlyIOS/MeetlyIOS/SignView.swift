//
//  SignView.swift
//  MeetlyIOS
//
//  Created by Niko Neufeld on 24.07.2024.
//

import SwiftUI
import AuthenticationServices
import Alamofire
struct SignView: View {
    var body: some View {
        VStack {
            Text("Log in!").font(.title)
            SignIn().font(.headline).frame(height:50).padding()
        
        }
    }
}
struct SignIn: View {
    var body: some View {
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
                    let signInWithJwt = LogInWithJwt(jwt: jwtToken, givenName: givenName,familyName: familyName,email: email)
                    AF.request("http://10.0.1.245:3000/signin_apple",
                               method: .post,
                               parameters: signInWithJwt,
                               encoder: JSONParameterEncoder.default).response { response in
                        debugPrint(response)
                    }
                
                    
                   // print(userCredential.identityToken)

                }
            case .failure(let error):
                print("Could not authenticate: \\(error.localizedDescription)")
            }

        }
    }
}
#Preview {
    SignView()
}
