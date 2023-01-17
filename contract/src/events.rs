pub mod events_cep47;
pub mod events_cep78;

use crate::TokenIdentifier;

pub(crate) enum Event {
    Cep47(events_cep47::CEP47Event),
    Cep47Dict(events_cep47::CEP47Event),
    Cep78(TokenIdentifier, events_cep78::CEP78Event),
}

pub(crate) fn record_event(event_enum: Event) {
    match event_enum {
        Event::Cep47(event) => events_cep47::record_event(&event),
        Event::Cep47Dict(event) => events_cep47::record_event_dictionary(&event),
        Event::Cep78(token_identifier, event) => {
            events_cep78::record_event(token_identifier, event)
        }
    }
}
