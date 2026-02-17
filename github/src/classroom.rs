use crate::Client;
use crate::ClientResult;

pub struct Classroom {
    pub client: Client,
}

impl Classroom {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Classroom { client }
    }

    /**
     * Get an assignment.
     *
     * This function performs a `GET` to the `/assignments/{assignment_id}` endpoint.
     *
     * Gets a GitHub Classroom assignment. Assignment will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#get-an-assignment>
     *
     * **Parameters:**
     *
     * * `assignment_id: i64` -- The unique identifier of the classroom assignment.
     */
    pub async fn get_an_assignment(
        &self,
        assignment_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ClassroomAssignment>> {
        let url = self.client.url(
            &format!(
                "/assignments/{}",
                crate::progenitor_support::encode_path(&assignment_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List accepted assignments for an assignment.
     *
     * This function performs a `GET` to the `/assignments/{assignment_id}/accepted_assignments` endpoint.
     *
     * Lists any assignment repositories that have been created by students accepting a GitHub Classroom assignment. Accepted assignments will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#list-accepted-assignments-for-an-assignment>
     *
     * **Parameters:**
     *
     * * `assignment_id: i64` -- The unique identifier of the classroom assignment.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_accepted_assignments_for_an_assignment(
        &self,
        assignment_id: i64,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ClassroomAcceptedAssignment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/assignments/{}/accepted_assignments?{}",
                crate::progenitor_support::encode_path(&assignment_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List accepted assignments for an assignment.
     *
     * This function performs a `GET` to the `/assignments/{assignment_id}/accepted_assignments` endpoint.
     *
     * As opposed to `list_accepted_assignments_for_an_assignment`, this function returns all the pages of the request at once.
     *
     * Lists any assignment repositories that have been created by students accepting a GitHub Classroom assignment. Accepted assignments will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#list-accepted-assignments-for-an-assignment>
     */
    pub async fn list_all_accepted_assignments_for_an_assignment(
        &self,
        assignment_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ClassroomAcceptedAssignment>>> {
        let url = self.client.url(
            &format!(
                "/assignments/{}/accepted_assignments",
                crate::progenitor_support::encode_path(&assignment_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get assignment grades.
     *
     * This function performs a `GET` to the `/assignments/{assignment_id}/grades` endpoint.
     *
     * Gets grades for a GitHub Classroom assignment. Grades will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#get-assignment-grades>
     *
     * **Parameters:**
     *
     * * `assignment_id: i64` -- The unique identifier of the classroom assignment.
     */
    pub async fn get_assignment_grades(
        &self,
        assignment_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ClassroomAssignmentGrade>>> {
        let url = self.client.url(
            &format!(
                "/assignments/{}/grades",
                crate::progenitor_support::encode_path(&assignment_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get assignment grades.
     *
     * This function performs a `GET` to the `/assignments/{assignment_id}/grades` endpoint.
     *
     * As opposed to `get_assignment_grades`, this function returns all the pages of the request at once.
     *
     * Gets grades for a GitHub Classroom assignment. Grades will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#get-assignment-grades>
     */
    pub async fn get_all_assignment_grades(
        &self,
        assignment_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ClassroomAssignmentGrade>>> {
        let url = self.client.url(
            &format!(
                "/assignments/{}/grades",
                crate::progenitor_support::encode_path(&assignment_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List classrooms.
     *
     * This function performs a `GET` to the `/classrooms` endpoint.
     *
     * Lists GitHub Classroom classrooms for the current user. Classrooms will only be returned if the current user is an administrator of one or more GitHub Classrooms.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#list-classrooms>
     *
     * **Parameters:**
     *
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_classrooms(
        &self,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleClassroom>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/classrooms?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List classrooms.
     *
     * This function performs a `GET` to the `/classrooms` endpoint.
     *
     * As opposed to `list_classrooms`, this function returns all the pages of the request at once.
     *
     * Lists GitHub Classroom classrooms for the current user. Classrooms will only be returned if the current user is an administrator of one or more GitHub Classrooms.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#list-classrooms>
     */
    pub async fn list_all_classrooms(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleClassroom>>> {
        let url = self.client.url(&"/classrooms".to_string(), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a classroom.
     *
     * This function performs a `GET` to the `/classrooms/{classroom_id}` endpoint.
     *
     * Gets a GitHub Classroom classroom for the current user. Classroom will only be returned if the current user is an administrator of the GitHub Classroom.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#get-a-classroom>
     *
     * **Parameters:**
     *
     * * `classroom_id: i64` -- The unique identifier of the classroom.
     */
    pub async fn get_a_classroom(
        &self,
        classroom_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Classroom>> {
        let url = self.client.url(
            &format!(
                "/classrooms/{}",
                crate::progenitor_support::encode_path(&classroom_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List assignments for a classroom.
     *
     * This function performs a `GET` to the `/classrooms/{classroom_id}/assignments` endpoint.
     *
     * Lists GitHub Classroom assignments for a classroom. Assignments will only be returned if the current user is an administrator of the GitHub Classroom.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#list-assignments-for-a-classroom>
     *
     * **Parameters:**
     *
     * * `classroom_id: i64` -- The unique identifier of the classroom.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_assignments_for_a_classroom(
        &self,
        classroom_id: i64,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleClassroomAssignment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/classrooms/{}/assignments?{}",
                crate::progenitor_support::encode_path(&classroom_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List assignments for a classroom.
     *
     * This function performs a `GET` to the `/classrooms/{classroom_id}/assignments` endpoint.
     *
     * As opposed to `list_assignments_for_a_classroom`, this function returns all the pages of the request at once.
     *
     * Lists GitHub Classroom assignments for a classroom. Assignments will only be returned if the current user is an administrator of the GitHub Classroom.
     *
     * FROM: <https://docs.github.com/rest/classroom/classroom#list-assignments-for-a-classroom>
     */
    pub async fn list_all_assignments_for_a_classroom(
        &self,
        classroom_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleClassroomAssignment>>> {
        let url = self.client.url(
            &format!(
                "/classrooms/{}/assignments",
                crate::progenitor_support::encode_path(&classroom_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
