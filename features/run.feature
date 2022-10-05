Feature: run a command in all folders

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: it runs the given command in all subfolders

    Scenario: happy path
      When running "m run pwd"
      Then it prints:
        """
        step 0: cd {{examples_dir}}/go

        step 1: run pwd
        {{examples_dir}}/go

        step 2: cd {{examples_dir}}/go_node

        step 3: run pwd
        {{examples_dir}}/go_node

        step 4: cd {{examples_dir}}/node

        step 5: run pwd
        {{examples_dir}}/node

        ALL DONE
        """
      And it returns "success"
      And there is no saved state

    Scenario: command not found
      When running "m run zonk"
      Then it prints:
        """
        step 0: cd {{examples_dir}}/go

        step 1: run zonk
        ERROR: command "zonk" not found
        """
      And there is no saved state

  Rule: does not interrupt an existing walk

    Scenario: within an existing walk
      Given I am in the middle of running "m walk"
      When running "m run pwd"
      Then it prints:
        """
        Running in all 3 folders.

        step 2: cd {{examples_dir}}/go_node
        step 3: exit
        step 4: cd {{examples_dir}}/node
        step 5: exit
        step 6: cd {{examples_dir}}

        ERROR: a session is already active. Please abort this currently running session before starting a new one.
        """
      And the saved state is unchanged
