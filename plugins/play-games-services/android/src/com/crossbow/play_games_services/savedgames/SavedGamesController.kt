package com.crossbow.play_games_services.savedgames

import android.app.Activity
import android.util.Log
import android.util.Pair
import com.google.android.gms.games.PlayGames
import com.google.android.gms.games.SnapshotsClient
import com.google.android.gms.games.SnapshotsClient.DataOrConflict
import com.google.android.gms.games.snapshot.Snapshot
import com.google.android.gms.games.snapshot.SnapshotMetadataChange
import com.google.android.gms.tasks.Continuation
import java.io.IOException

class SavedGamesController(
    private val activity: Activity,
    private val savedGamesListener: SavedGamesListener,
) {
    companion object {
        const val RC_SAVED_GAMES = 9009
    }

    fun showSavedGamesUI(
        title: String,
        allowAddBtn: Boolean,
        allowDeleteBtn: Boolean,
        maxNumberOfSavedGamesToShow: Int,
    ) {
        PlayGames.getSnapshotsClient(activity)
            .getSelectSnapshotIntent(
                title,
                allowAddBtn,
                allowDeleteBtn,
                maxNumberOfSavedGamesToShow,
            )
            .addOnSuccessListener { activity.startActivityForResult(it, RC_SAVED_GAMES) }
            .addOnFailureListener { savedGamesListener.onSavedGameFailed() }
    }

    private fun writeSnapshot(snapshot: Snapshot, data: ByteArray, description: String) {
        snapshot.snapshotContents.writeBytes(data)
        val metadata = SnapshotMetadataChange.Builder().setDescription(description).build()
        PlayGames.getSnapshotsClient(activity).commitAndClose(snapshot, metadata)
            .addOnSuccessListener { savedGamesListener.onSavedGameSuccess() }
            .addOnFailureListener { savedGamesListener.onSavedGameFailed() }
    }

    fun saveSnapshot(gameName: String, dataToSave: String, description: String) {
        PlayGames.getSnapshotsClient(activity)
            .open(gameName, true, SnapshotsClient.RESOLUTION_POLICY_MOST_RECENTLY_MODIFIED)
            .addOnFailureListener { savedGamesListener.onSavedGameFailed() }
            .continueWith<Pair<Snapshot, ByteArray>>(
                Continuation<DataOrConflict<Snapshot>, Pair<Snapshot, ByteArray>> { task ->
                    task.result?.data?.let { Pair(it, dataToSave.toByteArray()) }
                }
            )
            .addOnCompleteListener { task ->
                val result = if (task.isSuccessful) task.result else null
                if (result == null) {
                    savedGamesListener.onSavedGameFailed()
                } else {
                    writeSnapshot(result.first, result.second, description)
                }
            }
    }

    fun loadSnapshot(gameName: String) {
        PlayGames.getSnapshotsClient(activity)
            .open(gameName, true, SnapshotsClient.RESOLUTION_POLICY_MOST_RECENTLY_MODIFIED)
            .addOnFailureListener { savedGamesListener.onSavedGameLoadFailed() }
            .continueWith<ByteArray>(Continuation { task ->
                try {
                    task.result?.data?.snapshotContents?.readFully()
                } catch (error: IOException) {
                    Log.e("SavedGamesController", "Error while reading snapshot.", error)
                    null
                }
            })
            .addOnCompleteListener { task ->
                val data = if (task.isSuccessful) task.result else null
                if (data == null) {
                    savedGamesListener.onSavedGameLoadFailed()
                } else {
                    savedGamesListener.onSavedGameLoadSuccess(String(data))
                }
            }
    }

    fun createNewSnapshot(currentSaveName: String) {
        savedGamesListener.onSavedGameCreateSnapshot(currentSaveName)
    }
}
