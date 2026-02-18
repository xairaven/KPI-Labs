using DAL.EF;
using DAL.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.ChangeTracking;
using Moq;

namespace DAL.Tests;

public class BaseRepositoryUnitTests
{
    [Fact]
    public void Create_InputEmployeeInstance_CalledAddMethodOfDBSetWithEmployeeInstance()
    {
        // Arrange
        DbContextOptions opt = new DbContextOptionsBuilder<TransportSystemContext>()
            .Options;
        var mockContext = new Mock<TransportSystemContext>(opt);
        var mockDbSet = new Mock<DbSet<Employee>>();
        mockContext
            .Setup(context =>
                context.Set<Employee>(
                ))
            .Returns(mockDbSet.Object);
        var repository = new TestEmployeeRepository(mockContext.Object);
        
        var expectedEmployee = new Mock<Employee>().Object;
        
        //Act
        repository.Create(expectedEmployee);
        
        // Assert
        mockDbSet.Verify(
            dbSet => dbSet.Add(
                expectedEmployee
            ), Times.Once);
    }
    
    [Fact]
    public void Get_InputId_CalledFindMethodOfDBSetWithCorrectId()
    {
        // Arrange
        DbContextOptions opt = new DbContextOptionsBuilder<TransportSystemContext>()
            .Options;
        var mockContext = new Mock<TransportSystemContext>(opt);
        var mockDbSet = new Mock<DbSet<Employee>>();
        mockContext
            .Setup(context =>
                context.Set<Employee>(
                ))
            .Returns(mockDbSet.Object);
        
        // Random id.
        var expectedEmployee = new Employee { Id = 1 };
        mockDbSet.Setup(dbSet => dbSet.Find(expectedEmployee.Id))
            .Returns(expectedEmployee);
        
        var repository = new TestEmployeeRepository(mockContext.Object);
        
        // Act
        var actualEmployee = repository.Get(expectedEmployee.Id);
        
        // Assert
        mockDbSet.Verify(
            dbSet => dbSet.Find(
                expectedEmployee.Id
            ), Times.Once());
        Assert.Equal(expectedEmployee, actualEmployee);
    }
    
    [Fact]
    public void Delete_InputId_CalledFindAndRemoveMethodsOfDBSetWithCorrectArg()
    {
        // Arrange
        DbContextOptions opt = new DbContextOptionsBuilder<TransportSystemContext>()
            .Options;
        var mockContext = new Mock<TransportSystemContext>(opt);
        var mockDbSet = new Mock<DbSet<Employee>>();
        mockContext
            .Setup(context =>
                context.Set<Employee>(
                ))
            .Returns(mockDbSet.Object);
        
        // Random id.
        var expectedEmployee = new Employee { Id = 1 };
        mockDbSet.Setup(mock => mock.Find(expectedEmployee.Id))
            .Returns(expectedEmployee);
        
        var repository = new TestEmployeeRepository(mockContext.Object);
        
        // Act
        repository.Delete(expectedEmployee.Id);
        
        // Assert
        mockDbSet.Verify(
            dbSet => dbSet.Remove(
                expectedEmployee
            ), Times.Once);
    }
}