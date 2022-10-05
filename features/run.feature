Feature: running a command automatically

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Scenario: valid binary, no saved state
    When running "m run pwd"
    Then it prints:
      """
      step 0: cd {{examples_dir}}/go1

      step 1: run pwd
      {{examples_dir}}/go1

      step 2: cd {{examples_dir}}/node1

      step 3: run pwd
      {{examples_dir}}/node1

      step 4: cd {{examples_dir}}/node2

      step 5: run pwd
      {{examples_dir}}/node2

      ALL DONE
      """
    And it returns "success"
    And the saved state is now:
      """
      {
        "rootDir": null,
        "steps": [],
        "folders": null
      }
      """

  @this
  Scenario: non-existing binary
    When running "m run zonk"
    Then it prints:
      """
      step 0: cd /home/kevlar/mrt/examples/simple/go1

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

        step 2: cd /home/kevlar/mrt/examples/simple/node1
        step 3: exit
        step 4: cd /home/kevlar/mrt/examples/simple/node2
        step 5: exit
        step 6: cd /home/kevlar/mrt/examples/simple

        ERROR: a session is already active. Please abort this currently running session before starting a new one.
        """
      And the saved state is unchanged
