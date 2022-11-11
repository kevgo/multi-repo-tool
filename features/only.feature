Feature: "only" command

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: "m only" reduces the folder set to matching folders

    Scenario: limiting using "m only"
      When running "m only test -f package.json"
      Then it prints:
        """
        ...

        Limiting execution to 2/3 top-level folders:
        1. {{examples_dir}}/go_node
        2. {{examples_dir}}/node
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1/4: cd {{examples_dir}}/go_node

        step 2/4: run pwd
        {{examples_dir}}/go_node

        step 3/4: cd {{examples_dir}}/node

        step 4/4: run pwd
        {{examples_dir}}/node

        ALL DONE
        """

  Rule: subsequent limits add to existing limits

    Scenario: nested limiting
      When running "m only test -f package.json"
      Then it prints:
        """
        ...

        Limiting execution to 2/3 top-level folders:
        1. {{examples_dir}}/go_node
        2. {{examples_dir}}/node
        """
      And it returns "success"
      When running "m only test -f go.mod"
      Then it prints:
        """
        ..

        Tightening the existing limit of 2/3 top-level folders further to 1/3 top-level folders:
        1. {{examples_dir}}/go_node
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1/2: cd {{examples_dir}}/go_node

        step 2/2: run pwd
        {{examples_dir}}/go_node

        ALL DONE
        """

  Rule: does not allow empty folder sets

    Scenario: limiting all folders
      When running "m only test -f zonk"
      Then it prints:
        """
        ...

        ERROR: all folders have been filtered out
        """
      And it returns "failure"
      And there is no saved state

  Rule: displays guidance when calling without condition

    Scenario: call without condition
      When running "m only"
      Then it prints:
        """
        ERROR: missing condition

        The "only" command filters the currently active directories. It keeps those in which the given CLI command returns exit code 0.

        You forgot to tell me the CLI command I should run in each directory. You do it like this:

          m only <command>

        As an example, to select all directories that contain a Node.js codebase:

          m only test -f package.json
        """
      And it returns "failure"
