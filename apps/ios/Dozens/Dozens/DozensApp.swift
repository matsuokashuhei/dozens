//
//  DozensApp.swift
//  Dozens
//
//  Created by Shuhei Matsuoka on 2026/10/12.
//

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
