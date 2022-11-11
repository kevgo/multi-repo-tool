Feature: Clone an entire GitHub organization

  Scenario: no org to clone provided
    Given I am in the "simple" example folder
    When running "m clone"
    Then it prints:
      """
      ERROR: missing GitHub organization to clone

      The clone command clones all repositories in a GitHub organization onto your machine.
      You need to tell me which GitHub organization to clone.
      You do it like this:

        m clone <GitHub org name>

      Example:

        m clone github.com/kevgo
      """
    And it returns "failure"
