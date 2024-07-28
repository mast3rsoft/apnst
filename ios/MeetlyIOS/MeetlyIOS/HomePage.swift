//
//  HomePage.swift
//  MeetlyIOS
//
//  Created by Niko Neufeld on 22.07.2024.
//

import SwiftUI

struct HomePage: View {
    var jwtToken = ""
    @State private var selectedTab = "MYEVENTS"
    private func addItme() {
        print("Yay! I am A ")
    }
    var body: some View {
        NavigationStack {
            TabView(selection: $selectedTab) {
                DiscoverEventsView()
                    .badge(2)
                    .tabItem {
                        Label("Discover", systemImage: "globe")
                    }.tag("DISC")
                Text("")
                    .tabItem {
                        Label("My Events", systemImage: "calendar")
                    }.tag("MYEVENTS")
                Text("Acc")
                    .badge("!")
                    .tabItem {
                        Label("Account", systemImage: "person.crop.circle.fill")
                    }.tag("ACC")
                
            }.navigationTitle("Home")
                .toolbar {
                    if selectedTab == "MYEVENTS" {
                        ToolbarItem(placement: .automatic) {
                            NavigationLink {
                                CreateEventView(jwtToken: jwtToken)
                            } label: {
                                Label("", systemImage: "plus")
                            }
                        }
                    }
                    
                }
        }
        }
}

#Preview {
    HomePage()
}
