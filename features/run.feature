Feature: running a command automatically

  Background:
    Given I am in the "simple" example folder

  Scenario: valid binary
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

  Scenario: non-executable binary

  Scenario: non-existing binary
