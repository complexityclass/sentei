//
//  core.swift
//  Sentei
//
//  Created by Valerii Popov on 24/03/2024.
//

import Foundation
import SharedTypes

@MainActor
class Core: ObservableObject {
    @Published var view: ViewModel
    
    init() {
        self.view = try! .bincodeDeserialize(input: [UInt8](Sentei.view()))
    }
    
    func update(_ event: Event) {
        let effects = [UInt8](processEvent(Data(try! event.bincodeSerialize())))
        
        let requests: [Request] = try! .bincodeDeserialize(input: effects)
        for request in requests {
            processEffect(request)
        }
    }
    
    func processEffect(_ request: Request) {
        switch request.effect {
        case .render:
            view = try! .bincodeDeserialize(input: [UInt8](Sentei.view()))
        }
    }
}

extension ViewModel {
    var move: String {
        "\(self.row):\(self.col)"
    }
}
