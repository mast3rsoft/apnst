//
//  CreateEventView.swift
//  MeetlyIOS
//
//  Created by Niko Neufeld on 22.07.2024.
//

import SwiftUI
import Alamofire
import AuthenticationServices
struct CreateEventView: View {
    @Environment(\.dismiss) private var dismiss
    @State var title = ""
    @State var description = ""
    @State var isPublic = false
    var jwtToken = ""
    var body: some View {
        Form {
                Section(header: Text("Basic Information")) {
                    TextField("Title", text: $title)
                    TextField("Description", text: $description)
                    Toggle(isOn: $isPublic) {
                        Text("Public")
                    }
                    }
                    Button("Submit") {
                        createEvent()
                        dismiss()
                    }
            
        }.navigationTitle("Create an Event")
    
        
    }
    func createEvent() {
        let event = Event(id: 0, title: title, desc: description, public: isPublic)
        let createReq = CreateEventPost(event: event)
        let headers: HTTPHeaders = [
                .authorization(bearerToken: jwtToken),
            ]
        AF.request("http://localhost:3000/create_event",
                   
                   method: .post,
                   parameters: createReq,
                   encoder: JSONParameterEncoder.default, headers: headers).response { response in
            debugPrint(response)
        }
    }
}

#Preview {
    CreateEventView()
}
