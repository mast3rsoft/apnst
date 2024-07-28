//
//  DiscoverEventsView.swift
//  MeetlyIOS
//
//  Created by Niko Neufeld on 22.07.2024.
//

import SwiftUI
import Alamofire
struct DiscoverEventsView: View {
    @State var events: Array<Event> = []
    
    var body: some View {
        List {
            ForEach(events) { event in
                VStack(alignment: .leading, spacing: 3) {
                           Text(event.title)
                               .foregroundColor(.primary)
                               .font(.headline)
                    HStack {
                               Label(event.desc, systemImage: "text.justify.leading").truncationMode(.tail)
                                Spacer()
                               Button(action: {
                                   
                               }) {
                                   Image(systemName: "check").foregroundStyle(.blue).font(.headline)

                               }.frame(alignment: .trailing)
                           }
                           .foregroundColor(.secondary)
                           .font(.subheadline)
                       }
            }
        }.onAppear {
            withAnimation {
                var _ = AF.request("http://localhost:3000/discover_events").responseDecodable(of: DiscoverResponse.self) { response in
                    if case .success(let r) = response.result {
                        events = r.resp
                    }
                }
            }
        } .refreshable {
            withAnimation {
                var _ = AF.request("http://localhost:3000/discover_events").responseDecodable(of: DiscoverResponse.self) { response in
                    if case .success(let r) = response.result {
                        events = r.resp
                    }
                }
            }
        }
    }
}

#Preview {
    DiscoverEventsView()
}
