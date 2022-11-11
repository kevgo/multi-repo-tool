Feature: run a command in all folders

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: it runs the given command in all subfolders

    Scenario: happy path
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
      And it returns "success"
      And I am now in the "simple" example folder
      And there is no saved state

    Scenario: command not found
      When running "m run zonk"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/go

        step 2/6: run zonk
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
        ERROR: a session is already active. Please abort this currently running session before starting a new one.

        Running in all 3 folders.

        step 3/7: cd {{examples_dir}}/go_node
        step 4/7: exit
        step 5/7: cd {{examples_dir}}/node
        step 6/7: exit
        step 7/7: cd {{examples_dir}}
        """
      And it returns "failure"
      And the saved state is unchanged

  Rule: "m retry" retries a failed step

    Scenario: a step fails in a subdirectory
      When running "m run test -z foo"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/go

        step 2/6: run test -z foo
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "go" subfolder

      When running "m retry"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/go

        step 2/6: run test -z foo
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am still in the "go" subfolder

      When running "m retry"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/go

        step 2/6: run test -z foo
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am still in the "go" subfolder

  Rule: "m ignore" ignores a failed step

    Scenario:
      When running "m run test -z foo"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/go

        step 2/6: run test -z foo
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "go" subfolder

      When running "m ignore"
      Then it prints:
        """
        step 3/6: cd {{examples_dir}}/go_node

        step 4/6: run test -z foo
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "go_node" subfolder

      When running "m ignore"
      Then it prints:
        """
        step 5/6: cd {{examples_dir}}/node

        step 6/6: run test -z foo
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "node" subfolder

      When running "m ignore"
      Then it prints:
        """
        ALL DONE
        """
      And it returns "success"
      And I am now back in the "simple" example folder

  Rule: "m ignore-all" ignores all failing steps

    Scenario: ignoring all failures
      When running "m run test -z foo"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/go

        step 2/6: run test -z foo
        ERROR: Abort, Retry, Ignore?
        """
      And it returns "failure"
      And I am now in the "go" subfolder

      When running "m ignore-all"
      Then it prints:
        """
        step 3/6: cd {{examples_dir}}/go_node

        step 4/6: run test -z foo

        step 5/6: cd {{examples_dir}}/node

        step 6/6: run test -z foo

        ALL DONE
        """
      And it returns "success"
      And I am now back in the "simple" example folder

  Rule: displays help when calling without arguments

    Scenario: calling without command to run
      When running "m run"
      Then it prints:
        """
        ERROR: missing command to run

        The "run" command executes the given CLI command in all currently active directories.

        You forgot to tell me the CLI command I should run in each directory. You do it like this:

          m run <command>

        As an example, to display the path of all active directories:

          m run pwd
        """
      And it returns "failure"
