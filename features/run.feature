Feature: run a command in all folders

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: it runs the given command in all subfolders

    Scenario: happy path
      When running "m run pwd"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go

        step 2: run pwd
        {{examples_dir}}/go

        step 3: cd {{examples_dir}}/go_node

        step 4: run pwd
        {{examples_dir}}/go_node

        step 5: cd {{examples_dir}}/node

        step 6: run pwd
        {{examples_dir}}/node

        ALL DONE
        """
      And it returns "success"
      And I am now in the "simple" example folder
      And there is no saved state

    Scenario: command not found
      When running "m run zonk"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go

        step 2: run zonk
        ERROR: command "zonk" not found
        """
      And it returns "failure"
      And there is no saved state

  Rule: does not interrupt an existing walk

    Scenario: within an existing walk
      Given I am in the middle of running "m walk"
      When running "m run pwd"
      Then it prints:
        """
        Running in all 3 folders.

        step 3: cd {{examples_dir}}/go_node
        step 4: exit
        step 5: cd {{examples_dir}}/node
        step 6: exit
        step 7: cd {{examples_dir}}

        ERROR: a session is already active. Please abort this currently running session before starting a new one.
        """
      And it returns "failure"
      And the saved state is unchanged

  Rule: "m retry" retries a failed step

    Scenario: a step fails in a subdirectory
      When running "m run ls zonk"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go

        step 2: run ls zonk
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "go" subfolder

      When running "m retry"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go

        step 2: run ls zonk
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am still in the "go" subfolder

      When running "m retry"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go

        step 2: run ls zonk
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am still in the "go" subfolder

  Rule: "m ignore" ignores a failed step

    Scenario:
      When running "m run ls zonk"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go

        step 2: run ls zonk
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "go" subfolder

      When running "m ignore"
      Then it prints:
        """
        step 3: cd {{examples_dir}}/go_node

        step 4: run ls zonk
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "go_node" subfolder

      When running "m ignore"
      Then it prints:
        """
        step 5: cd {{examples_dir}}/node

        step 6: run ls zonk
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "node" subfolder

      When running "m ignore"
      Then it prints:
        """
        """
      And it returns "success"
      And I am now back in the "simple" example folder

  Rule: "m ignore-all" ignores all failing steps
