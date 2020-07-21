This section covers client flows and details of the API endpoints. The URI layout of the new API is structured to support a rich authentication and authorization model by leveraging namespaces. All endpoints will be prefixed by the API version and the repository name:

/v2/<name>/
For example, an API endpoint that will work with the library/ubuntu repository, the URI prefix will be:

/v2/library/ubuntu/
This scheme provides rich access control over various operations and methods using the URI prefix and http methods that can be controlled in variety of ways.

Classically, repository names have always been two path components where each path component is less than 30 characters. The V2 registry API does not enforce this. The rules for a repository name are as follows:

A repository name is broken up into path components. A component of a repository name MUST begin with one or more lowercase alpha-numeric characters. Subsequent lowercase alpha-numeric characters are OPTIONAL and MAY be separated by periods, dashes or underscores. More strictly, it MUST match the regular expression [a-z0-9]+(?:[._-][a-z0-9]+)*.

If a repository name has two or more path components, they MUST be separated by a forward slash ("/").
The total length of a repository name, including slashes, MUST be less than 256 characters.
These name requirements only apply to the registry API and SHOULD accept a superset of what is supported by other components.

All endpoints SHOULD support aggressive http caching, compression and range headers, where appropriate. The new API attempts to leverage HTTP semantics where possible but MAY break from standards to implement targeted features.


Method	Path	Entity	Description
GET	/v2/	Base	Check that the endpoint implements distribution API.
GET	/v2/<name>/tags/list	Tags	Fetch the tags under the repository identified by name.
GET	/v2/<name>/manifests/<reference>	Manifest	Fetch the manifest identified by name and reference where reference can be a tag or digest. A HEAD request can also be issued to this endpoint to obtain resource information without receiving all data.
PUT	/v2/<name>/manifests/<reference>	Manifest	Put the manifest identified by name and reference where reference can be a tag or digest.
DELETE	/v2/<name>/manifests/<reference>	Manifest	Delete the manifest identified by name and reference. Note that a manifest can only be deleted by digest.
GET	/v2/<name>/blobs/<digest>	Blob	Retrieve the blob from the registry identified by digest. A HEAD request can also be issued to this endpoint to obtain resource information without receiving all data.
DELETE	/v2/<name>/blobs/<digest>	Blob	Delete the blob identified by name and digest
POST	/v2/<name>/blobs/uploads/	Initiate Blob Upload	Initiate a resumable blob upload. If successful, an upload location will be provided to complete the upload. Optionally, if the digest parameter is present, the request body will be used to complete the upload in a single request.
GET	/v2/<name>/blobs/uploads/<session_id>	Blob Upload	Retrieve status of upload identified by session_id. The primary purpose of this endpoint is to resolve the current status of a resumable upload.
PATCH	/v2/<name>/blobs/uploads/<session_id>	Blob Upload	Upload a chunk of data for the specified upload.
PUT	/v2/<name>/blobs/uploads/<session_id>	Blob Upload	Complete the upload specified by session_id, optionally appending the body as the final chunk.
DELETE	/v2/<name>/blobs/uploads/<session_id>	Blob Upload	Cancel outstanding upload processes, releasing associated resources. If this is not called, the unfinished uploads will eventually timeout.
