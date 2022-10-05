Feature: manually iterate all folders

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: it goes to the next subdirectory

    Scenario: multiple subdirectories
      When running "m walk"
      Then it prints:
        """
        step 0: cd {{examples_dir}}/go1
        """
      And I am now in the "go1" subfolder
      And it returns "success"
      When running "m next"
      Then it prints:
        """
        step 2: cd {{examples_dir}}/node1
        """
      And I am now in the "node1" subfolder
      And it returns "success"
      When running "m next"
      Then it prints:
        """
        step 4: cd {{examples_dir}}/node2
        """
      And I am now in the "node2" subfolder
      And it returns "success"
      When running "m next"
      Then it prints:
        """
        step 6: cd {{examples_dir}}

        ALL DONE
        """
      And I am now back in the "simple" example folder
      And it returns "success"
      And there is no saved state
