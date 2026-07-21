// This file is part of the uutils awk package.
//
// For the full copyright and license information, please view the LICENSE
// files that was distributed with this source code.

use std::{
    io::{Result, Write, stdout},
    process::exit,
};

use interpreter::{Bytecode, Interpreter, IoRequest, IoResponse, Signal};

use crate::cli::ArgQueueItem;

pub struct AwkRt<'a> {
    intrp: Interpreter<'a>,
    bc: &'a Bytecode<'a>,
    queue: &'a [ArgQueueItem],
}

impl<'a> AwkRt<'a> {
    pub fn new(intrp: Interpreter<'a>, bc: &'a Bytecode<'a>, queue: &'a [ArgQueueItem]) -> Self {
        Self { intrp, bc, queue }
    }

    pub fn main_event_loop(&mut self) -> Result<()> {
        let res = self.begin_event_loop().and_then(|_| self.rule_event_loop());
        self.end_event_loop(0).and(res)
    }

    pub fn begin_event_loop(&mut self) -> Result<()> {
        let range = self.bc.begin_code();
        let mut sig = self.intrp.run_code(self.bc, range.clone())?;
        loop {
            let req = match sig {
                Signal::Suspend(req) => req,
                Signal::End => break,
                Signal::Next => todo!(),
                Signal::NextFile => todo!(),
                Signal::Exit(code) => return self.end_event_loop(code),
            };
            let res = self.perform_io(&req);
            sig = self.intrp.resume(self.bc, range.clone(), req, res)?;
        }
        Ok(())
    }

    pub fn end_event_loop(&mut self, exit_code: i32) -> Result<()> {
        let range = self.bc.end_code();
        let mut sig = self.intrp.run_code(self.bc, range.clone())?;
        loop {
            let req = match sig {
                Signal::Suspend(req) => req,
                Signal::End => break,
                Signal::Exit(code) => exit(code),
                Signal::Next | Signal::NextFile => unreachable!(),
            };
            let res = self.perform_io(&req);
            sig = self.intrp.resume(self.bc, range.clone(), req, res)?;
        }
        exit(exit_code)
    }

    pub fn rule_event_loop(&mut self) -> Result<()> {
        let range = self.bc.rules_code();

        while let Some(_item) = self.queue.split_off_first() {
            let mut sig = self.intrp.run_code(self.bc, range.clone())?;
            loop {
                let req = match sig {
                    Signal::Suspend(req) => req,
                    Signal::End => break,
                    Signal::Next => todo!(),
                    Signal::NextFile => todo!(),
                    Signal::Exit(code) => return self.end_event_loop(code),
                };
                let res = self.perform_io(&req);
                sig = self.intrp.resume(self.bc, range.clone(), req, res)?;
            }
        }
        Ok(())
    }

    fn perform_io(&mut self, req: &IoRequest) -> Result<IoResponse> {
        match req {
            IoRequest::WriteStdout(buf) => {
                stdout().lock().write_all(buf).map(|_| IoResponse::Empty)
            }
        }
    }
}
