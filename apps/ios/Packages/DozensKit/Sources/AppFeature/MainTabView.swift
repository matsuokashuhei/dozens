import Application
import DesignSystem
import Domain
import SwiftUI

struct MainTabView: View {
    enum Tab: Hashable {
        case timeline, search, record, notifications, account
    }

    let user: User
    let session: SessionStore
    @State private var selection: Tab = .timeline

    var body: some View {
        TabView(selection: $selection) {
            SwiftUI.Tab(String(localized: "Timeline", bundle: .module), systemImage: "house", value: .timeline) {
                ComingSoonView(title: String(localized: "Timeline", bundle: .module), systemImage: "house")
            }
            SwiftUI.Tab(String(localized: "Record", bundle: .module), systemImage: "record.circle", value: .record) {
                ComingSoonView(title: String(localized: "Record", bundle: .module), systemImage: "record.circle")
            }
            SwiftUI.Tab(String(localized: "Notifications", bundle: .module), systemImage: "bell", value: .notifications) {
                ComingSoonView(title: String(localized: "Notifications", bundle: .module), systemImage: "bell")
            }
            SwiftUI.Tab(String(localized: "Account", bundle: .module), systemImage: "person", value: .account) {
                AccountView(user: user, session: session)
            }
            SwiftUI.Tab(value: .search, role: .search) {
                ComingSoonView(title: String(localized: "Search", bundle: .module), systemImage: "magnifyingglass")
            }
        }
        .tabBarMinimizeBehavior(.onScrollDown)
    }
}

private struct ComingSoonView: View {
    let title: String
    let systemImage: String

    var body: some View {
        NavigationStack {
            ContentUnavailableView {
                Label(title, systemImage: systemImage)
            } description: {
                Text("This screen is being migrated from the previous app.", bundle: .module)
            }
            .navigationTitle(title)
            .background(.dozensBackground)
        }
    }
}
