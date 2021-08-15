use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

const RECORDED_MEETINGS: &str = r#"{
  "from": "2021-07-16",
  "to": "2021-08-15",
  "page_size": 30,
  "total_records": 1,
  "next_page_token": "",
  "meetings": [
    {
      "uuid": "asdasd",
      "id": 4,
      "account_id": "test",
      "host_id": "asdasdasd",
      "topic": "Jessie Frazelle's Personal Meeting Room",
      "type": 4,
      "start_time": "2021-08-15T01:52:41Z",
      "timezone": "America/Los_Angeles",
      "host_email": "jess@thing.com",
      "duration": 0,
      "total_size": 1026074,
      "recording_count": 3,
      "share_url": "https://zoom.us/rec/share/",
      "recording_files": [
        {
          "id": "0b3de97c-4260-436e685",
          "meeting_id": "asdasdasd",
          "recording_start": "2021-08-15T01:53:44Z",
          "recording_end": "2021-08-15T01:53:57Z",
          "file_type": "M4A",
          "file_extension": "M4A",
          "file_size": 212576,
          "play_url": "https://zoom.us/rec/play/x-At.0hXu",
          "download_url": "https://zoom.us/rec/download/exImJm",
          "status": "completed",
          "recording_type": "audio_only"
        },
        {
          "id": "a60b54fb5d36",
          "meeting_id": "PsY==",
          "recording_start": "2021-08-15T01:53:44Z",
          "recording_end": "2021-08-15T01:53:57Z",
          "file_type": "MP4",
          "file_extension": "MP4",
          "file_size": 813436,
          "play_url": "https://zoom.us/rec/play/TjV6mGTlmLcsn1sau5CjZT",
          "download_url": "https://zoom.us/rec/download/jV6mGTlmLOmUClk.e-r0UnKVmtS5CjZT",
          "status": "completed",
          "recording_type": "shared_screen_with_speaker_view"
        },
        {
          "id": "f85388a",
          "meeting_id": "PsYwQ33E==",
          "recording_start": "2021-08-15T01:53:44Z",
          "recording_end": "2021-08-15T01:53:57Z",
          "file_type": "CHAT",
          "file_extension": "TXT",
          "file_size": 62,
          "play_url": "https://zoom.us/rec/play/lW0Ow4STWXtxsgHbST5mHC1f.Nse",
          "download_url": "https://zoom.us/rec/download/lW0Ow4SrLHbST5mHxXa4cV",
          "status": "completed",
          "recording_type": "chat_file"
        }
      ]
    }
  ]
}"#;

#[test]
fn test_deserialize_recorded_meetings() {
    let deserialized: crate::types::GetAccountCloudRecordingResponse =
        serde_json::from_str(RECORDED_MEETINGS).unwrap();
    println!("recorded_meetings = {:?}", deserialized);

    assert_eq!(
        DateTime::<Utc>::from_utc(
            NaiveDateTime::new(
                NaiveDate::parse_from_str("2021-07-16", "%Y-%m-%d").unwrap(),
                NaiveTime::from_hms(0, 0, 0),
            ),
            Utc,
        ),
        deserialized.from.unwrap()
    );
}
