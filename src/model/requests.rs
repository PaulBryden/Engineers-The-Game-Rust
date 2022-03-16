use super::super::pathfinding::pathfinder::TilePosition;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Default)]
pub struct SpriteMoveRequest {
    pub tick: u32,
    pub sprite_uuid: u32,
    pub position: TilePosition,
}

#[derive(Copy, Clone)]
pub enum Request {
    SpriteMove(SpriteMoveRequest),
}

#[derive(Clone, Default)]
pub struct RequestQueue {
    requests: Vec<Request>,
}

impl RequestQueue {
    pub fn AddRequest(&mut self, request: Request) {
        self.requests.push(request);
        self.requests.sort_by(|a, b| {
            RequestQueue::GetTickOfParticularRequest(a)
                .cmp(&RequestQueue::GetTickOfParticularRequest(b))
        });
    }

    pub fn GetTickOfParticularRequest(request: &Request) -> u32 {
        let mut tick = 0;
        match request {
            Request::SpriteMove(sprite_move_request) => tick = sprite_move_request.tick,
        }
        return tick;
    }
    pub fn GetCopyOfNextRequestToBeProcessed(&self) -> Option<Request> {
        if self.requests.len() > 0 {
            return Some(self.requests[0]);
        } else {
            return None;
        }
    }
    pub fn PopNextRequestToBeProcessed(&mut self) -> Option<Request> {
        if self.requests.len() > 0 {
            Some(self.requests.remove(0))
        } else {
            return None;
        }
    }

    pub fn GetNumberOfRequests(&self) -> usize {
        return self.requests.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn TestRequestSorting() {
        let mut request_queue: RequestQueue = RequestQueue::default();
        let request_1: Request = Request::SpriteMove(SpriteMoveRequest {
            tick: 13,
            sprite_uuid: 12,
            position: TilePosition { x: 0, y: 0 },
        });
        let request_2: Request = Request::SpriteMove(SpriteMoveRequest {
            tick: 17,
            sprite_uuid: 113232,
            position: TilePosition { x: 0, y: 0 },
        });
        let mut uuid_holding_variable_1: u32 = 1;
        let mut uuid_holding_variable_2: u32 = 2;
        request_queue.AddRequest(request_2);
        assert_eq!(request_queue.GetNumberOfRequests() == 1, true);
        request_queue.AddRequest(request_1);
        match request_queue.GetCopyOfNextRequestToBeProcessed().unwrap() {
            Request::SpriteMove(sprite_move_request) => {
                uuid_holding_variable_1 = sprite_move_request.tick
            }
        }
        match request_1 {
            Request::SpriteMove(sprite_move_request) => {
                uuid_holding_variable_2 = sprite_move_request.tick
            }
        }
        assert_eq!(uuid_holding_variable_1 == uuid_holding_variable_2, true);

        assert_eq!(request_queue.GetNumberOfRequests() == 2, true);
        request_queue.PopNextRequestToBeProcessed();
        assert_eq!(request_queue.GetNumberOfRequests() == 1, true);
        match request_queue.GetCopyOfNextRequestToBeProcessed().unwrap() {
            Request::SpriteMove(sprite_move_request) => {
                uuid_holding_variable_1 = sprite_move_request.tick
            }
        }
        match request_2 {
            Request::SpriteMove(sprite_move_request) => {
                uuid_holding_variable_2 = sprite_move_request.tick
            }
        }
        assert_eq!(uuid_holding_variable_1 == uuid_holding_variable_2, true);
    }
}
