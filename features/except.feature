Feature: "except" command

  Rule: "m except" reduces the folder set to non-matching folders

    Scenario: limiting using "m except"
      Given I am in the "simple" example folder
      And no mrt configuration
      When running "m except test -f package.json"
      Then it prints:
        """
        ...

        Limiting execution to 1/3 top-level folders:
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

    @this
    Scenario: call without command
      Given I am in the "simple" example folder
      And no mrt configuration
      When running "m except"
      Then it prints:
        """
        ERROR: missing command

        You need to provide a command that I can run to determine which folders you want me to iterate through.
        """
      And it returns "failure"
