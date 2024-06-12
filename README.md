# EasyPeasyDB

### Storage side
- [X] B-tree index
- [ ] B+ tree table
- [ ] Pager

### Compiler side
- [ ] Tokenzier
- [ ] Parser
- [ ] Code-gen

### Possible additions (not in current scope)
- Transactions
- Query optimization (join-ordering, etc)
- Durability & ACID compliance


## Planned architecture
                                                                               
                                                                               
                                                                               
     ┌────────────────────────────────────────────────────────────────────┐    
     │  SQL compiler                                                      │    
     │                                                                    │    
     │      ┌───────────────┐    ┌─────────────┐  ┌─────────────────┐     │    
     │      │               │    │             │  │                 │     │    
     │      │               │    │             │  │                 │     │    
     │      │   Tokenizer   │    │   Parser    │  │ Code Generator  │     │    
     │      │               │    │             │  │                 │     │    
     │      └───────────────┘    └─────────────┘  └─────────────────┘     │    
     │                                                                    │    
     └────────────────────────────────────────────────────────────────────┘    
                                                                               
     ┌────────────────────────────────────────────────────────────────────┐    
     │                                                                    │    
     │       ┌───────────────┐   ┌────────────┐  ┌─────────────┐          │    
     │       │               │   │            │  │             │          │    
     │       │     B-tree    │   │   Pager    │  │OS-interface │          │    
     │       │               │   │            │  │             │          │    
     │       └───────────────┘   └────────────┘  └─────────────┘          │    
     │                                                                    │    
     │ Storage engine                                                     │    
     └────────────────────────────────────────────────────────────────────┘    
                                                                               



                                                                               
