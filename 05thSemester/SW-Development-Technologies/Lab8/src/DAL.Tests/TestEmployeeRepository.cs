using DAL.Entities;
using DAL.Repositories.Impl;
using Microsoft.EntityFrameworkCore;

namespace DAL.Tests;

internal class TestEmployeeRepository(DbContext context) 
    : BaseRepository<Employee>(context);