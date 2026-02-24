Handle ctrlc failure
 - Ensure proper cleanup of resources and actual termination.
Reorder the input of arguments to prevent ambiguity
    i.e if a search comes up empty; let it be empty not a non-exisent file
        `rfgrep jibrish search`
        results in:
            r#
                Searching 0 files...
                No matches found 
            #
    - This should fail and indicate why.
Establish all code paths that are simply inconsequential and remove them.
    - This has to be a methodical process.
Filter searches to be done on certain files with a given set of extensions.
    - This will require a new argument in the CLI command.
Handle pipelining of rfgrep to output.
    - Ensure rfgrep's out put can be directly used by another program that is using it.
Make search be the default method when no instruction is specified.

[NOTE:] Do not forget to remove this file from the tree when done.
