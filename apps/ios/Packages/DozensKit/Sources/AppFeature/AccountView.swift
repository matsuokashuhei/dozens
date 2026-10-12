import Application
import DesignSystem
import Domain
import SwiftUI

struct AccountView: View {
    let user: User
    let session: SessionStore
    @State private var isConfirmingSignOut = false
    @State private var isSigningOut = false

    var body: some View {
        NavigationStack {
            List {
                Section {
                    LabeledContent {
                        Text(user.name)
                    } label: {
                        Text("Username", bundle: .module)
                    }
                    .accessibilityIdentifier("account.username")
                }
                Section {
                    Button(role: .destructive) {
                        isConfirmingSignOut = true
                    } label: {
                        if isSigningOut {
                            ProgressView()
                        } else {
                            Text("Sign Out", bundle: .module)
                        }
                    }
                    .disabled(isSigningOut)
                    .accessibilityIdentifier("account.signOut")
                }
            }
            .scrollContentBackground(.hidden)
            .background(.dozensBackground)
            .navigationTitle(Text("Account", bundle: .module))
            .confirmationDialog(
                Text("Sign Out", bundle: .module),
                isPresented: $isConfirmingSignOut,
                titleVisibility: .visible
            ) {
                Button(role: .destructive) {
                    Task {
                        isSigningOut = true
                        await session.signOut()
                        isSigningOut = false
                    }
                } label: {
                    Text("Sign Out", bundle: .module)
                }
            } message: {
                Text("Are you sure you want to sign out?", bundle: .module)
            }
        }
    }
}
