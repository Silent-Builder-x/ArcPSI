use arcis::*;

#[encrypted]
mod psi_engine {
    use arcis::*;

    pub struct UserContacts {
        // 用户本地通讯录中的哈希列表 (批量处理 4 个)
        pub queries: [u64; 4], 
    }

    pub struct GlobalRegistry {
        // 链上已注册用户的哈希池 (模拟 4 个槽位)
        pub registered_users: [u64; 4],
    }

    pub struct IntersectionResult {
        // 布尔掩码：对应 UserContacts 的顺序
        // 1 = 该联系人在平台上, 0 = 未注册
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

        // 执行 O(N*M) 的全盲比对
        // 这种嵌套循环在 MPC 中会展开为并行比较门电路
        for i in 0..4 {
            let mut found = 0u64;
            
            // 将用户当前联系人 (i) 与全局库中所有用户 (j) 逐一比对
            for j in 0..4 {
                let is_match = user.queries[i] == global.registered_users[j];
                
                // 使用 Mux 累积状态：只要匹配到一次，found 就置为 1
                found = if is_match { 1u64 } else { found };
            }
            results[i] = found;
        }

        let result = IntersectionResult {
            matches: results,
        };

        // 结果仅返回给发起查询的用户
        user_ctxt.owner.from_arcis(result)
    }
}