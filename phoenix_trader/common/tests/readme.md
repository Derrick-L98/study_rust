# 集成测试
cargo test --test akaclient_test --  --exact --nocapture 
cargo test [--package akaclient] --test akaclient_test -- test_reloadcache --exact --nocapture 
# test_reloadcache 不存在于任务mod中,可以简化为
cargo test test_reloadcache -- exact --nocapture