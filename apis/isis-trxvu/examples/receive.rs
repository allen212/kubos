/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate isis_trxvu;

use isis_trxvu::Trxvu;

pub fn main() {
    let radio = match Trxvu::new() {
        Ok(r) => r,
        Err(e) => {
            print!("Err {}", e);
            return;
        }
    };

    println!("Receiving message");
    match radio.read() {
        Ok(d) => println!("Received {:?}", d),
        Err(e) => println!("{}", e),
    }
}
