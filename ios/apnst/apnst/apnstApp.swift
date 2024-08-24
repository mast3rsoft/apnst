//
//  apnstApp.swift
//  apnst
//
//  Created by Niko Neufeld on 07.08.2024.
//

import SwiftUI
import SwiftData

class MyAppDelegate: NSObject, UIApplicationDelegate, ObservableObject {
    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions:[UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        // Override point for customization after application launch.
        UIApplication.shared.registerForRemoteNotifications()
        print("here1")

        return true
    }


    func application(_ application: UIApplication, didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data) {
        print("here")
        let deviceTokenString = deviceToken.reduce("", { $0 + String(format: "%02X", $1) })
         print("deviceToken:\(deviceTokenString)")    }


    func application(_ application: UIApplication,
                     didFailToRegisterForRemoteNotificationsWithError
                     error: Error) {
        print("here2", error)

        // Try again later.
    }
    
    func application(_ application: UIApplication, didReceiveRemoteNotification userInfo: [AnyHashable : Any],
                     fetchCompletionHandler completionHandler: @escaping (UIBackgroundFetchResult) -> Void
    ) {
        print(userInfo)
        completionHandler(UIBackgroundFetchResult.newData)
        
    }
}


@main
struct apnstApp: App {
    @UIApplicationDelegateAdaptor private var appDelegate: MyAppDelegate
    var sharedModelContainer: ModelContainer = {
        let schema = Schema([
            Item.self,
        ])
        let modelConfiguration = ModelConfiguration(schema: schema, isStoredInMemoryOnly: false)

        do {
            return try ModelContainer(for: schema, configurations: [modelConfiguration])
        } catch {
            fatalError("Could not create ModelContainer: \(error)")
        }
    }()

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
        .modelContainer(sharedModelContainer)
    }
}

