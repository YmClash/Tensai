use crate::semantic::semantic::{Scope, SemanticAnalyzer, Symbol};

impl SemanticAnalyzer{

    //on  creee  un nouveau scope

    // fn enter_scope(&mut self) -> usize{
    //     let new_scope = Scope::new(Some(self.current_scope));
    //     let scope_id = self.symbol_table.add_scope(new_scope);
    //     self.current_scope = scope_id;
    //     scope_id
    // }
    //
    // //sortie du scope
    // fn exit_scope(&mut self){
    //     if let Some(parent) = self.symbol_table.get_scope(self.current_scope).parent {
    //         self.current_scope = parent;
    //     }
    // }
    //
    // //Verification sie un symbole existe dans le scope actuel
    // fn  lookup_symbol(&self, name:&str) -> Option<&Symbol>{
    //     self.symbol_table.lookup(name,self.current_scope)
    // }


}