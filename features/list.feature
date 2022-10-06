Feature: list subfolders matching a condition

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: it displays the folders where the condition returns 0

    Scenario: happy path
      When running "m list ls go.mod"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/go

        step 2/6: check ls go.mod
        go.mod

        step 3/6: cd {{examples_dir}}/go_node

        step 4/6: check ls go.mod
        go.mod

        step 5/6: cd {{examples_dir}}/node

        step 6/6: check ls go.mod
        """
      And it returns "success"
      And I am now in the "simple" example folder
      And there is no saved state
