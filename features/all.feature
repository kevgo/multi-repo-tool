Feature: "all command"

  Rule: "m all" removes all limits

    Scenario: limiting using "m only"
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
      When running "m all"
      Then it prints:
        """
        """
      And it returns "success"
      And there is no saved state
      When running "m run pwd"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/go

        step 2/6: run pwd
        {{examples_dir}}/go

        step 3/6: cd {{examples_dir}}/go_node

        step 4/6: run pwd
        {{examples_dir}}/go_node

        step 5/6: cd {{examples_dir}}/node

        step 6/6: run pwd
        {{examples_dir}}/node

        ALL DONE
        """
