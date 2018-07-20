# P4Runtime specification documents

## CI upload of build documents

Travis takes care of uploading the built HTML version of the spec to AWS S3. The
latest working draft (master branch) can be found
[here](https://s3-us-west-2.amazonaws.com/p4runtime/docs/master/P4Runtime-Spec.html).

Additionally, you can access the HTML version of the spec for any given branch
of this repository by using the following URL:
https://s3-us-west-2.amazonaws.com/p4runtime/travis/<your_branch_nam>/P4Runtime-Spec.html.

Unfortunately, because of how Travis encrypts environment variables (which are
required to upload documents to S3), this does not work for branches in forked
repositories, even for opened pull requests.
