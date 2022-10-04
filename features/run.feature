Feature: running a command automatically

  Background:
    Given I am in the "examples" folder

  Scenario: running all folders
    When running "m run action"
    Then it prints:
      """
      step 0: cd /home/kevlar/mrt/examples/go1

      step 1: run action
      """
