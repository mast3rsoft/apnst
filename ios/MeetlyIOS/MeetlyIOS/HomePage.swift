//
//  HomePage.swift
//  MeetlyIOS
//
//  Created by Niko Neufeld on 22.07.2024.
//

import SwiftUI

struct HomePage: View {
    private func addItme() {
        print("Yay! I am A ")
    }
    var body: some View {
        NavigationStack {
            TabView {
                Text("Discover")
                    .badge(2)
                    .tabItem {
                        Label("Discover", systemImage: "globe")
                    }
                Text("")
                    .tabItem {
                        Label("My Events", systemImage: "calendar")
                    }
                Text("Acc")
                    .badge("!")
                    .tabItem {
                        Label("Account", systemImage: "person.crop.circle.fill")
                    }
            }
        }
    }
}

#Preview {
    HomePage()
}
