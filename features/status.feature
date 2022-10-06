Feature: display the current status

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Scenario: no status
    When running "m status"
    Then it prints:
      """
      Running in all folders.

      I'm not doing anything right now.
      """

  Scenario: within a walk
    Given I am in the middle of running "m walk"
    When running "m status"
    Then it prints:
      """
      Running in all 3 folders.

      step 3: cd {{examples_dir}}/go_node
      step 4: exit
      step 5: cd {{examples_dir}}/node
      step 6: exit
      step 7: cd {{examples_dir}}
      """
