# tests asset for svn-diff

come from `subversion\tests\libsvn_diff\parse-diff-test.c`


+ property_and_text_uidiff.patch: Add edge cases like context lines stripped from leading whitespaces that starts with 'Added: ', 'Deleted: ' or 'Modified: '
+ diff_symbols_in_prop_unidiff.patch: A unidiff containing diff symbols in the body of the hunks.