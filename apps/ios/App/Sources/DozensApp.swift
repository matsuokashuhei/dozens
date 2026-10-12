import AppFeature
import SwiftUI

@main
struct DozensApp: App {
    var body: some Scene {
        WindowGroup {
            AppRootView(configuration: .fromEnvironment())
        }
    }
}
