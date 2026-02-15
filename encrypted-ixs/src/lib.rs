use arcis::*;

#[encrypted]
mod psi_engine {
    use arcis::*;

    pub struct UserContacts {
        // Hash list in the user's local address book (batch process 4)
        pub queries: [u64; 4], 
    }

    pub struct GlobalRegistry {
        // Hash pool of registered users on-chain (simulate 4 slots)
        pub registered_users: [u64; 4],
    }

    pub struct IntersectionResult {
        // Boolean mask: corresponds to the order of UserContacts
        // 1 = This contact is on the platform, 0 = Not registered
        pub matches: [u64; 4],
    }

    #[instruction]
    pub fn perform_psi(
        user_ctxt: Enc<Shared, UserContacts>,
        registry_ctxt: Enc<Shared, GlobalRegistry>
    ) -> Enc<Shared, IntersectionResult> {
        let user = user_ctxt.to_arcis();
        let global = registry_ctxt.to_arcis();
        
        let mut results = [0u64; 4];

        // Perform O(N*M) fully blind comparison
        // This nested loop will expand into parallel comparison gates in MPC
        for i in 0..4 {
            let mut found = 0u64;
            
            // Compare the current user contact (i) with all users (j) in the global registry
            for j in 0..4 {
                let is_match = user.queries[i] == global.registered_users[j];
                
                // Use Mux to accumulate state: as long as a match is found, set found to 1
                found = if is_match { 1u64 } else { found };
            }
            results[i] = found;
        }

        let result = IntersectionResult {
            matches: results,
        };

        // The result is only returned to the user who initiated the query
        user_ctxt.owner.from_arcis(result)
    }
}