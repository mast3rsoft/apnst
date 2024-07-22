//
//  CreateEventView.swift
//  MeetlyIOS
//
//  Created by Niko Neufeld on 22.07.2024.
//

import SwiftUI

struct CreateEventView: View {
    @Environment(\.dismiss) private var dismiss
    @State var title = ""
    @State var description = ""
    var body: some View {
        Form {
                Section(header: Text("Basic Information")) {
                    TextField("Title", text: $title)
                    TextField("Description", text: $description)

                    }
                    Button("Submit") {
                        dismiss()
                    }
        }.navigationTitle("Create an Event")
    
        
    }
}

#Preview {
    CreateEventView()
}
