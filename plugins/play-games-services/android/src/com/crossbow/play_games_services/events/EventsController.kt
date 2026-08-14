package com.crossbow.play_games_services.events

import android.app.Activity
import com.google.android.gms.games.PlayGames
import com.google.android.gms.games.event.Event
import org.json.JSONArray
import org.json.JSONObject

class EventsController(
    private val activity: Activity,
    private val eventsListener: EventsListener,
) {
    fun submitEvent(eventId: String, incrementBy: Int) {
        try {
            PlayGames.getEventsClient(activity).increment(eventId, incrementBy)
            eventsListener.onEventSubmitted(eventId)
        } catch (_: RuntimeException) {
            eventsListener.onEventSubmittingFailed(eventId)
        }
    }

    fun loadEvents() = load { PlayGames.getEventsClient(activity).load(true) }

    fun loadEventById(eventIds: Array<String>) =
        load { PlayGames.getEventsClient(activity).loadByIds(true, *eventIds) }

    private fun load(request: () -> com.google.android.gms.tasks.Task<com.google.android.gms.games.AnnotatedData<com.google.android.gms.games.event.EventBuffer>>) {
        request().addOnCompleteListener { task ->
            val events = if (task.isSuccessful) task.result?.get() else null
            if (events == null) {
                eventsListener.onEventsLoadingFailed()
                return@addOnCompleteListener
            }
            if (events.count == 0) {
                eventsListener.onEventsEmpty()
            } else {
                val json = JSONArray()
                events.forEach { json.put(eventInfo(it)) }
                eventsListener.onEventsLoaded(json.toString())
            }
        }
    }

    private fun eventInfo(event: Event) = JSONObject().apply {
        put("id", event.eventId)
        put("name", event.name)
        put("value", event.value)
        put("description", event.description)
        put("imgUrl", event.iconImageUrl)
    }
}
