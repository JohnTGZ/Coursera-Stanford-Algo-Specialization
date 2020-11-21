def knapsackBig(items, knapsackSize):
  cache ={}

  #main recursive function to help fil lcache
  def bigHelper(item, size):
    #hashing tuples for this if-condition
    if (item, size) not in cache:
      if item == 0:
        cache[item, size] = 0
      elif items[item][1] > size:
        cache[item, size] = bigHelper(item -1, size)
      else:
        cache[item,size] = max( bigHelper(item-1, size),
                                bigHelper(item-1, size-items[item][1]) + items[item][0] )
    return cache[item,size]