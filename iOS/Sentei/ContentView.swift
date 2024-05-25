//
//  ContentView.swift
//  Sentei
//
//  Created by Valerii Popov on 24/03/2024.
//

import SharedTypes
import SwiftUI

struct ContentView: View {
    @ObservedObject var core: Core
    
    var body: some View {
        VStack {
            Text("\(core.view.isWhite ? "white" : "black")")
            Text(core.view.move)
            HStack {
                ActionButton(label: "Reset", color: .red) {
                    core.update(.makeMove)
                }
            }
        }
    }
}

struct ActionButton: View {
    var label: String
    var color: Color
    var action: () -> Void
    
    init(label: String, color: Color, action: @escaping () -> Void) {
        self.label = label
        self.color = color
        self.action = action
    }
    
    var body: some View {
        Button(action: action) {
            Text(label)
                .fontWeight(.bold)
                .font(.body)
                .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
                .background(color)
                .cornerRadius(10)
                .foregroundColor(.white)
                .padding()
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(core: Core())
    }
}

