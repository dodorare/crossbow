package com.crossbow.play_games_services.events

interface EventsListener {
    fun onEventSubmitted(eventId: String)
    fun onEventSubmittingFailed(eventId: String)
    fun onEventsLoaded(eventsJson: String)
    fun onEventsEmpty()
    fun onEventsLoadingFailed()
}
