use arcis::*;

#[encrypted]
mod psi_engine {
    use arcis::*;

    pub struct ContactSet {
        pub hashes: [u64; 3], 
    }

    pub struct DiscoveryResult {
        pub match_flags: [u64; 3], // 1 代表匹配成功, 0 代表无匹配
    }

    #[instruction]
    pub fn match_contacts(
        user_set_ctxt: Enc<Shared, ContactSet>,
        global_set_ctxt: Enc<Shared, ContactSet>
    ) -> Enc<Shared, DiscoveryResult> {
        let user_set = user_set_ctxt.to_arcis();
        let global_set = global_set_ctxt.to_arcis();
        
        let mut results = [0u64; 3];

        // 执行隐私集求交 (O(N*M) 嵌套比对)
        // Arcis 会将嵌套循环展开为并行的比较电路
        for i in 0..3 {
            let mut found = 0u64;
            for j in 0..3 {
                // 如果用户哈希与全局库哈希一致
                let is_match = user_set.hashes[i] == global_set.hashes[j];
                
                // 使用 V4 规范的 if-else Mux 逻辑更新状态
                found = if is_match { 1u64 } else { found };
            }
            results[i] = found;
        }

        let result = DiscoveryResult {
            match_flags: results,
        };

        user_set_ctxt.owner.from_arcis(result)
    }
}