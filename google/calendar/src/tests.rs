const EVENT: &str = r#"{
   "kind": "calendar#event",
   "etag": "\"3258434761704000\"",
   "id": "1hg20kmdhcj1er51ef0c7hkv3a_R20210806T170000",
   "status": "confirmed",
   "htmlLink": "https://www.google.com/calendar/event?eid=MWhnMjBrbWRoY2oxZXI1MWVmMGM3aGt2M2FfMjAyMTA4MDZUMTcwMDAwWiBkYXZpZEBveGlkZWNvbXB1dGVyLmNvbQ",
   "created": "2020-07-23T18:08:11.000Z",
   "updated": "2021-08-17T16:23:00.852Z",
   "summary": "Weekly Control Plane Huddle",
   "description": "This is the event for control plane huddles.\n\nYou can submit topics at: https://control-plane-huddle-form.corp.oxide.computer\n\nThe Airtable workspace lives at: https://control-plane-huddle.corp.oxide.computer\n\nDiscussion topics:\n- Storage demos from Alan Hanson\n- HTTP client generation within omicron from David Crespo",
   "location": "Warehouse-1-Burroughs (8) [Logitech Conferencing Camera]",
   "creator": {
    "email": "bryan@oxidecomputer.com"
   },
   "organizer": {
    "email": "david@oxidecomputer.com",
    "self": true
   },
   "start": {
    "dateTime": "2021-08-06T10:00:00-07:00",
    "timeZone": "America/Los_Angeles"
   },
   "end": {
    "dateTime": "2021-08-06T11:00:00-07:00",
    "timeZone": "America/Los_Angeles"
   },
   "recurrence": [
    "RRULE:FREQ=WEEKLY;BYDAY=FR"
   ],
   "iCalUID": "1hg20kmdhcj1er51ef0c7hkv3a_R20210806T170000@google.com",
   "sequence": 0,
   "attendees": [
    {
     "email": "bryan@oxidecomputer.com",
     "responseStatus": "accepted"
    },
    {
     "email": "david@oxidecomputer.com",
     "organizer": true,
     "self": true,
     "responseStatus": "accepted"
    },
    {
     "email": "all@oxidecomputer.com",
     "displayName": "all",
     "responseStatus": "needsAction"
    },
    {
     "email": "luqman@oxidecomputer.com",
     "responseStatus": "needsAction"
    },
    {
     "email": "oxidecomputer.com_1887kqo79i8eoh62lp0ro6mndmncc6gb6cp3ae9o68rj0d1o70@resource.calendar.google.com",
     "displayName": "Warehouse-1-Burroughs (8) [Logitech Conferencing Camera]",
     "resource": true,
     "responseStatus": "accepted"
    }
   ],
   "hangoutLink": "https://meet.google.com/njt-gxbf-wxc",
   "conferenceData": {
    "entryPoints": [
     {
      "entryPointType": "video",
      "uri": "https://meet.google.com/njt-gxbf-wxc",
      "label": "meet.google.com/njt-gxbf-wxc"
     },
     {
      "entryPointType": "more",
      "uri": "https://tel.meet/njt-gxbf-wxc?pin=9846914324958",
      "pin": "9846914324958"
     },
     {
      "regionCode": "US",
      "entryPointType": "phone",
      "uri": "tel:+1-513-788-1953",
      "label": "+1 513-788-1953",
      "pin": "256832845"
     }
    ],
    "conferenceSolution": {
     "key": {
      "type": "hangoutsMeet"
     },
     "name": "Google Meet",
     "iconUri": "https://fonts.gstatic.com/s/i/productlogos/meet_2020q4/v6/web-512dp/logo_meet_2020q4_color_2x_web_512dp.png"
    },
    "conferenceId": "njt-gxbf-wxc",
    "signature": "ADXwMqMkgRoMcehX/O3t+Xy0zuQZ"
   },
   "guestsCanModify": true,
   "reminders": {
    "useDefault": true
   },
   "eventType": "default"
}"#;

#[test]
fn test_deserialize() {
    let deserialized: crate::types::Event = serde_json::from_str(EVENT).unwrap();
    println!("event = {:?}", deserialized);
}
