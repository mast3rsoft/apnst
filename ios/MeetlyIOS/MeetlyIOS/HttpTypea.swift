//
//  HttpTypea.swift
//  MeetlyIOS
//
//  Created by Niko Neufeld on 23.07.2024.
//

import Foundation

struct Event: Decodable, Identifiable, Encodable {
    var id: Int
    var title: String
    var desc: String
    var `public`: Bool
}
struct DiscoverResponse: Decodable {
    var resp: Array<Event>
}
struct CreateEventPost: Encodable {
    var event: Event
}
struct LogInWithJwt: Encodable, Decodable {
    var jwt: String
    var givenName: String?
    var familyName: String?
    var email: String?
}
