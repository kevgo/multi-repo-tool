Feature: limiting folders

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: "m only" reduces the folder set to matching folders

    @this
    Scenario: limiting using "m only"
      When running "m only -- test -f package.json"
      Then it prints:
        """
        Limiting execution to 2/3 folders:
        1. {{examples_dir}}/go_node
        2. {{examples_dir}}/node
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go_node

        step 2: run pwd
        {{examples_dir}}/go_node

        step 3: cd {{examples_dir}}/node

        step 4: run pwd
        {{examples_dir}}/node

        ALL DONE
        """

  Rule: "m except" reduces the folder set to non-matching folders

    Scenario: limiting using "m except"
      When running "m except test -f package.json"
      Then it prints:
        """
        Limiting execution to 1/3 folders:
        1. {{examples_dir}}/go
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go

        step 2: run pwd
        {{examples_dir}}/go

        ALL DONE
        """

  Rule: subsequent limits add to existing limits

    Scenario: nested limiting
      When running "m only test -f package.json"
      Then it prints:
        """
        Limiting execution to 2/3 folders:
        1. {{examples_dir}}/go_node
        2. {{examples_dir}}/node
        """
      And it returns "success"
      When running "m only ls go.mod"
      Then it prints:
        """
        go.mod

        Tightening the existing limit of 2/3 folders further to 1/3 folders:
        1. {{examples_dir}}/go_node
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go_node

        step 2: run pwd
        {{examples_dir}}/go_node

        ALL DONE
        """

  Rule: does not allow empty folder sets

    Scenario: limiting all folders
      When running "m only ls zonk"
      Then it prints:
        """
        ERROR: all folders have been filtered out
        """
      And it returns "failure"
      And there is no saved state

  Rule: "m all" removes all limits

    Scenario: limiting using "m only"
      When running "m except test -f package.json"
      Then it prints:
        """
        Limiting execution to 1/3 folders:
        1. {{examples_dir}}/go
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1: cd {{examples_dir}}/go

        step 2: run pwd
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
