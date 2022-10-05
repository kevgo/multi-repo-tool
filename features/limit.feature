Feature: limiting folders

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: "m only" reduces the folder set to matching folders

    Scenario: limiting using "m only"
      When running "m only ls package.json"
      Then it prints:
        """
        package.json
        package.json

        Limiting execution to 2/3 folders:
        1. {{examples_dir}}/go_node
        2. {{examples_dir}}/node
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 0: cd {{examples_dir}}/go_node

        step 1: run pwd
        {{examples_dir}}/go_node

        step 2: cd {{examples_dir}}/node

        step 3: run pwd
        {{examples_dir}}/node

        ALL DONE
        """
      And it returns "success"

  Rule: "m except" reduces the folder set to non-matching folders

    Scenario: limiting using "m except"
      When running "m except ls package.json"
      Then it prints:
        """
        package.json
        package.json

        Limiting execution to 1/3 folders:
        1. {{examples_dir}}/go
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 0: cd {{examples_dir}}/go

        step 1: run pwd
        {{examples_dir}}/go

        ALL DONE
        """
      And it returns "success"

  Rule: subsequent limits build on previous limits

    Scenario: nested limiting
      When running "m only ls package.json"
      Then it prints:
        """
        package.json
        package.json

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
        step 0: cd {{examples_dir}}/go_node

        step 1: run pwd
        {{examples_dir}}/go_node

        ALL DONE
        """
      And it returns "success"

  Rule: does not allow empty folder sets

    @this
    Scenario: limiting all folders
      When running "m only ls zonk"
      Then it prints:
        """
        ERROR: all folders have been filtered out
        """
      And it returns "failure"
      And there is no saved state