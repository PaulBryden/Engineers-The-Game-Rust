use super::super::pathfinding::pathfinder::TilePosition;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug, Copy, Clone, Default)]
pub struct SpriteMoveRequest {
    pub tick: u32,
    pub sprite_uuid: u32,
    pub position: TilePosition,
}
#[derive(Serialize, Deserialize,Copy, Clone)]
pub enum SpriteType {
    Engineer,
}

#[derive(Serialize, Deserialize,Copy, Clone)]
pub struct SpriteCreateRequest {
    pub tick: u32,
    pub sprite_uuid: u32,
    pub sprite_type: SpriteType,
    pub position: TilePosition,
}

#[derive(Serialize, Deserialize,Copy, Clone)]
pub enum Request {
    SpriteMove(SpriteMoveRequest),
    SpriteCreate(SpriteCreateRequest),
}
pub trait RequestImpl {
    fn get_tick(&self) -> u32;
}
impl RequestImpl for Request {
    fn get_tick(&self) -> u32 {
        match self {
            Request::SpriteMove(sprite_move_request) => sprite_move_request.tick,
            Request::SpriteCreate(sprite_create_request) => sprite_create_request.tick,
        }
    }
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
        return request.get_tick();
    }
    pub fn GetRequestsOfParticularTick(&self, tick: u32) -> Vec<Request> {
        let mut requests = Vec::new();
        for request in &self.requests {
            if (request.get_tick() == tick) {
                requests.push(request.clone())
            }
        }
        return requests;
    }
    pub fn PurgeRequestsOlderThanTick(&mut self, tick: u32) {
        self.requests=self.requests.clone().into_iter().filter(|w| w.get_tick()>=tick).collect::<Vec<_>>();
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
        let mut returned_requests=request_queue.GetRequestsOfParticularTick(request_1.get_tick());
        assert_eq!(returned_requests.len() == 1, true);
        assert_eq!(returned_requests[0].get_tick() == 13, true);
        assert_eq!(request_queue.GetNumberOfRequests() == 2, true);
        request_queue.PurgeRequestsOlderThanTick(14);
        assert_eq!(request_queue.GetNumberOfRequests() == 1, true);
        returned_requests=request_queue.GetRequestsOfParticularTick(request_2.get_tick());
        assert_eq!(returned_requests.len() == 1, true);
        assert_eq!(returned_requests[0].get_tick() == request_2.get_tick(), true);
    }
}
