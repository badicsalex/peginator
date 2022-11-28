// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

pub trait FakeColored: Sized {
    fn bold(self) -> Self {
        self
    }
    fn red(self) -> Self {
        self
    }
    fn white(self) -> Self {
        self
    }
    fn blue(self) -> Self {
        self
    }
    fn cyan(self) -> Self {
        self
    }
    fn green(self) -> Self {
        self
    }
    fn yellow(self) -> Self {
        self
    }
}

impl FakeColored for &str {}

impl FakeColored for String {}
