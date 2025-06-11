# tests asset for svn-diff

come from `subversion\tests\libsvn_diff\parse-diff-test.c`


+ bad_git_diff_header.patch: Only the last git diff header is valid. The other ones either miss a path element or have noise between lines that must be continuous. See issue #3809.
+ property_and_text_uidiff.patch: Add edge cases like context lines stripped from leading whitespaces that starts with 'Added: ', 'Deleted: ' or 'Modified: '
+ diff_symbols_in_prop_unidiff.patch: A unidiff containing diff symbols in the body of the hunks.
+ path_with_spaces_unidiff.patch: A unidiff containing paths with spaces. 
+ unidiff_lacking_trailing_eol.patch: Don't add NL after this line