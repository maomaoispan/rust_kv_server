mod common_service;

pub mod abi;
pub mod command_request;
pub mod command_response;
pub mod commands;
pub mod kvpair;
pub mod value;
pub use common_service::CommandService;

#[cfg(test)]
mod tests {
    use crate::{service::dispatch, CommandRequest, CommandResponse, Kvpair, MemTable, Value};

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();

        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[]);
    }

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();

        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    #[test]
    fn hdel_should_work() {
        let store = MemTable::new();

        let cmd = CommandRequest::new_hset("score", "d1", 10.into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hdel("score", "d1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);

        let cmd = CommandRequest::new_hdel("score", "d2");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[], &[])
    }

    #[test]
    fn hget_with_non_exist_key_should_return_404() {
        let store = MemTable::new();

        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_error(res, 404, "Not found");
    }

    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 11.into()),
            CommandRequest::new_hset("score", "u2", 12.into()),
            CommandRequest::new_hset("score", "u3", 13.into()),
            CommandRequest::new_hset("score", "u1", 14.into()),
        ];

        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 14.into()),
            Kvpair::new("u2", 12.into()),
            Kvpair::new("u3", 13.into()),
        ];
        assert_res_ok(res, &[], pairs);
    }

    fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
        res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "");
        assert_eq!(res.values, values);
        assert_eq!(res.pairs, pairs);
    }

    fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
        assert_eq!(res.status, code);
        assert!(res.message.contains(msg));
        assert_eq!(res.values, &[]);
        assert_eq!(res.pairs, &[]);
    }
}
