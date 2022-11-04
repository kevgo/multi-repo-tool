Feature: "except" command

  Rule: "m except" reduces the folder set to non-matching folders

    Scenario: limiting using "m except"
      When running "m except test -f package.json"
      Then it prints:
        """
        ...

        Limiting execution to 1/3 folders:
        1. {{examples_dir}}/go
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1/2: cd {{examples_dir}}/go

        step 2/2: run pwd
        {{examples_dir}}/go

        ALL DONE
        """
